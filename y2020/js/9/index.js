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

    console.time(`Day 9: ${part}`);
    const preambleSize = 25;
    let badNum = 0;
    for (let i = 0; i < input.length - preambleSize; ++i)
    {
        const digits = new Set(input.slice(i, i+preambleSize));
        let found = false;
        for (const digit of digits)
        {
            if (digits.has(input[i+preambleSize] - digit)) found = true;
        }
        if (!found)
        {
            if (part === 'part-1')
            {
                console.timeEnd(`Day 9: ${part}`);
                console.log(`${input[i+preambleSize]} does not have a sum in the last 25 numbers`);
                return;
            }
            else
            {
                badNum = input[i+preambleSize];
                break;
            }
        }
    }
    if (part === 'part-2')
    {
        for (let i = 0; i < input.length; ++i)
        {
            let sum = 0;
            for (let j = i; j < input.length; ++j)
            {
                sum += input[j];
                if (badNum < sum)
                {
                    break;
                }
                else if (sum === badNum && i !== j)
                {
                    const min = Math.min(...input.slice(i,j+1));
                    const max = Math.max(...input.slice(i,j+1));
                    console.timeEnd(`Day 9: ${part}`);
                    console.log(`answer: ${min + max}`);
                    return;
                }
            }
        }
    }
    console.timeEnd(`Day 9: ${part}`);
    throw Error('No sum found.');
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
