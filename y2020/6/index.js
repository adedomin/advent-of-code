#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

// note bitwise ops are limited to 32bit words
// problem-space is 26bits (a-z)
function popcount32 (x)
{
    x -= x >> 1 & 0x55555555;
    x = (x & 0x33333333) + (x >> 2 & 0x33333333);
    x = x + (x >> 4) & 0x0f0f0f0f;
    x += x >> 8;
    x += x >> 16;
    return x & 0x7f;
}

async function main([ part, inputFile ])
{
    const input = (await readInput(inputFile /* stdin = undefined */))
        .split('\n\n');

    console.time(`Day-6: ${part}`);
    let yeses = 0;
    for (const group of input)
    {
        // 26 bitfield init to all 1's
        let total = 2**26-1;
        let yes = 0;
        for (const q of group)
        {
            if (q === '\n')
            {
                if (part === 'part-1')
                {
                    continue;
                }
                else
                {
                    total &= yes;
                    yes = 0;
                }
            }
            else
            {
                yes |= (1 << q.charCodeAt(0) - 97);
            }
        }
        if (part === 'part-1') total = yes;
        else                   total &= yes;
        yeses += popcount32(total);
    }
    console.timeEnd(`Day-6: ${part}`);
    console.log(`Number of Yes answers: ${yeses}`);
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
