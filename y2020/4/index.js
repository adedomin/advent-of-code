#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');
const passportParser = /(?<key>[^:\s]+):(?:[^:\s]+)\s?/gm;

/*
(?x)
(
  (?:cid:(?<cid>[^\s]*))
 |(?:byr:(?<byr>(19[2-9][0-9]|200[0-2])))
 |(?:iyr:(?<iyr>(201[0-9]|2020)))
 |(?:eyr:(?<eyr>(202[0-9]|2030)))
 |(?:hcl:(?<hcl>#[0-9a-fA-F]{6}))
 |(?:ecl:(?<ecl>amb|blu|brn|gry|grn|hzl|oth))
 |(?:pid:(?<pid>\d{9}))
 |(?:hgt:(?<hgt>(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in))
 |(?<ws>\s+)
 |(?<invalid>.)
)
 */
const passportTokenizer = /((?:cid:(?<cid>[^\s]*))|(?:byr:(?<byr>(19[2-9][0-9]|200[0-2])))|(?:iyr:(?<iyr>(201[0-9]|2020)))|(?:eyr:(?<eyr>(202[0-9]|2030)))|(?:hcl:(?<hcl>#[0-9a-fA-F]{6}))|(?:ecl:(?<ecl>amb|blu|brn|gry|grn|hzl|oth))|(?:pid:(?<pid>\d{9}))|(?:hgt:(?<hgt>(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in))|(?<ws>\s+)|(?<invalid>.))/gm;

function part1Token(p)
{
    const passport = new Set();
    for (const match of p.matchAll(passportParser))
    {
        passport.add(match.groups.key);
    }
    return passport.size === 8 || (passport.size === 7 && !passport.has('cid'));
}

function part2Token(p)
{
    const passport = new Set();
    for (const match of p.matchAll(passportTokenizer))
    {
        // groups has a null prototype, safe to iter with `in`
        for (const m in match.groups)
        {
            // groups get added explicity, even when undefined (non-match)
            if (match.groups[m] === undefined) continue;
            switch (m)
            {
            case 'byr':
            case 'iyr':
            case 'eyr':
            case 'hgt':
            case 'hcl':
            case 'ecl':
            case 'pid':
            case 'cid':
                passport.add(m);
                // console.log('regex assumed correct', m, match.groups[m]);
                break;
            case 'ws':
                break;
            case 'invalid':
                // console.log('invalid field value or other unexpected grammar', m, match.groups[m]);
                return false;
            }
        }
    }
    return passport.size === 8 || (passport.size === 7 && !passport.has('cid'));
}

async function main([ part, inputFile ])
{
    const input = (await readInput(inputFile /* stdin = undefined */));

    let tokenizer;
    if (part === 'part-1')      tokenizer = part1Token;
    else if (part === 'part-2') tokenizer = part2Token;
    else                        throw Error('Must be part-1 or part-2');
    // don't want to profile IO
    console.time(`Day-4: ${part}`);
    const numValid = input
        .split('\n\n')
        .reduce((curr, p) => curr + tokenizer(p), 0);
    console.timeEnd(`Day-4: ${part}`);
    console.log(`Valid Passports: ${numValid}`);
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
