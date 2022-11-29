#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

/*
(
  (?<value>(\d+|no other bags))
 |(?:(?<adjective>[a-zA-Z]+)\s
     (?<color>[\a-zA-Z]+)\s
     bags?)
 |(?<contain>contain)
 |(?<sep>,)
 |(?<stmt>\.)
 |(?<ws>\s+)
 |(?<invalid>.)
)
 */
const bagRuleTokenizer = /((?<value>(\d+|no other bags))|(?:(?<bag>[a-zA-Z]+\s[a-zA-Z]+)\sbags?)|(?<has>contain)|(?<sep>,)|(?<stmt>\.)|(?<ws>\s+)|(?<invalid>.))/gm;

function bagRuleParser(input)
{
    const grammar = new Map();
    let line = 1;
    /** @type {string[]} */
    let state = ['.start.'];

    for (const { groups } of input.matchAll(bagRuleTokenizer))
    {
        for (const m in groups)
        {
            if (groups[m] === undefined) continue;
            switch (m)
            {
            case 'value':
                if (state[0] !== '.value.' || state.length !== 2)
                {
                    throw SyntaxError(`value "${groups[m]}" unpaired with bag. line:${line}`);
                }
                else if (groups[m] === 'no other bags')
                {
                    state.push(0);
                    state.push('other');
                }
                else if (isNaN(+groups[m]))
                {
                    throw SyntaxError(`value "${groups[m]}" is not a number. line:${line}`);
                }
                else
                {
                    state.push(+groups[m]);
                }
                break;
            case 'bag':
                if (state[0] !== '.start.' && (isNaN(state[2]) || state.length !== 3))
                {
                    throw SyntaxError(`bag "${groups[m]}" must be at start or after a value. line:${line}`);
                }
                else
                {
                    state.push(groups[m]);
                }
                break;
            case 'has':
                if (state[0] !== '.start.' || state.length !== 2)
                {
                    throw SyntaxError(`"contain" separator must come after the first bag. line:${line}`);
                }
                else
                {
                    grammar.set(state[1], new Map());
                    state = [ '.value.', state[1] ];
                }
                break;
            case 'sep':
                if (state[0] !== '.value.' || state.length !== 4)
                {
                    throw SyntaxError(`Incomplete bag leaf "[${state.join(', ')}]" or leaf came before parent. line:${line}`);
                }
                else if (grammar.get(state[1]).has('other'))
                {
                    throw SyntaxError(`bag "${state[1]}" cannot contain other bags if it specifies "no other bags". line:${line}`);
                }
                else
                {
                    grammar.get(state[1]).set(state[3], state[2]);
                    state = state.slice(0, 2);
                }
                break;
            case 'stmt':
                if (state[0] !== '.value.' || state.length !== 4)
                {
                    throw SyntaxError(`Incomplete bag leaf "[${state.join(', ')}]" or leaf came before parent. line:${line}`);
                }
                else
                {
                    grammar.get(state[1]).set(state[3], state[2]);
                    state = [ '.start.' ];
                    ++line;
                }
                break;
            case 'ws':
                break;
            case 'invalid':
                throw SyntaxError(`Unknown symbol ${JSON.stringify(groups[m])} line:${line}`);
            }
        }
    }

    return grammar;
}

// added bonus, if it has a cycle, it will die with stack frame exhaustion.
/**
 * Find all permuation of colors containing bagToFind.
 *
 * @param {Map<string, Map<string, number>>} bagRules - the bag rules
 * @param {string} bagToFind - the bag to find
 * @param {string} bagCurr - the bag we are currently in
 * @return {number} number of found bags that can hold bagToFind.
 */
function findAllPermutations(bagRules, bagToFind, bagCurr)
{
    if (bagCurr === undefined)
    {
        let count = 0;
        for (const [ k ] of bagRules)
        {
            if (k === bagToFind) continue;
            count += findAllPermutations(bagRules, bagToFind, k);
        }
        return count;
    }
    else
    {
        if (bagRules.get(bagCurr).has(bagToFind))
        {
            return 1;
        }
        else
        {
            for (const [ k ] of bagRules.get(bagCurr))
            {
                if (k === 'other') continue;
                const found = findAllPermutations(bagRules, bagToFind, k);
                if (found) return found;
            }
            return 0;
        }
    }
}

// added bonus, if it has a cycle, it will die with stack frame exhaustion.
/**
 * Find a total count of bags we need for a given bag.
 *
 * @param {Map<string, Map<string, number>>} bagRules - the bag rules.
 * @param {string} topBag - the bag we need to find all bags we need.
 * @return {number} number of bags needed for this topBag + topBag
 */
function findTotalNeeded(bagRules, topBag)
{
    let count = 1; // ourself
    for (const [ bagName, numberOf ] of bagRules.get(topBag))
    {
        if (numberOf === 0) continue;
        const total = findTotalNeeded(bagRules, bagName);
        count += total * numberOf;
    }
    return count;
}

async function main([ part, inputFile ])
{
    const input = await readInput(inputFile /* stdin = undefined */);
    console.time(`Day 7: ${part}`);
    const bagRules = bagRuleParser(input);
    if (part === 'part-1')
    {
        const ans = findAllPermutations(bagRules, 'shiny gold');
        console.log(`Bags containing shiny gold bag: ${ans}`);
    }
    else
    {
        const ans = findTotalNeeded(bagRules, 'shiny gold');
        console.log(`Bags we need to contain: ${ans - 1}`);
    }
    console.timeEnd(`Day 7: ${part}`);
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
