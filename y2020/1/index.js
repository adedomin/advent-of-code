#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

async function main([ part, inputFile ])
{
    const input = (await readInput(inputFile))
        .split('\n')
        .map(n =>
        {
            return +n;
        });
    const sums = new Set(input);

    // profile
    console.time(`Day 1: ${part}`);
    let sum = 2020;
    if (input.includes('big')) sum = 99920044;

    if (part === 'part-1')
    {
        // PART 1
        for (const x of input)
        {
            if (sums.has(sum - x))
            {
                if ((sum - x) === x) continue;
                console.timeEnd(`Day 1: ${part}`);
                console.log(`answer: ${x} * ${sum - x} = ${x * (sum - x)}`);
                return;
            }
        }
    }
    else if (part === 'part-2')
    {
        // PART 2 (3SUM)
        for (const x of input)
        {
            for (const y of input)
            {
                if (sums.has(sum - (x + y)))
                {
                    console.timeEnd(`Day 1: ${part}`);
                    console.log(`answer: ${x} * ${y} * ${sum - (x+y)} = ${x * y * (sum - (x + y))}`);
                    return;
                }
            }
        }
    }
    else
    {
        // end profile
        console.timeEnd(`Day 1: ${part}`);
        throw Error('Must specify part-1 or part-2');
    }
    console.timeEnd(`Day 1: ${part}`);
    throw Error('No sum found.');
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
