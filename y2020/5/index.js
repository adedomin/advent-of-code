#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

async function main([ inputFile ])
{
    const input = (await readInput(inputFile /* stdin = undefined */))
        .split('\n');

    console.time('Day-5: part-1 / part-2');

    const planeSize = input[0].length;
    let max = -1;
    let min = 2**planeSize;
    let sum = 0;
    for (const pass of input)
    {
        let id = 0;
        for (let i = 0; i < planeSize; ++i)
        {
            // we pretend the string is valid and ignore F/L altogether...
            id = (id << 1) + (pass[i] === 'B' || pass[i] === 'R');
        }
        sum += id;
        if (id > max) max = id;
        if (id < min) min = id;
    }
    const mySeat = ((max + min) * (max - min + 1)) / 2 - sum;

    console.timeEnd('Day-5: part-1 / part-2');

    console.log(`Max Seat ID: ${max}`);
    console.log(`My Seat ID: ${mySeat}`);
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
