#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

class TerrainRight
{
    constructor(input)
    {
        this.map = input;
        this.curX = 0;
        this.curY = 0;
    }

    descend(x, y)
    {
        const newX = this.curX + x;
        const newY = this.curY + y;
        if (newY >= this.map.length) return undefined;
        this.curX = newX;
        this.curY = newY;
        return this.map[newY][newX % this.map[newY].length];
    }

    reset(x = 0, y = 0)
    {
        this.curX = x;
        this.curY = y;
    }
}


async function main([ part, inputFile ])
{
    const input = (await readInput(inputFile))
        .split('\n');

    console.time(`Day-3: ${part}`);
    const map = new TerrainRight(input);

    if (part === 'part-1')
    {
        let totalTrees = 0;
        let cell;
        while ((cell = map.descend(3, 1)) !== undefined)
        {
            if (cell === '#') ++totalTrees;
        }
        console.timeEnd(`Day-3: ${part}`);
        console.log(`Total Trees: ${totalTrees}`);
    }
    else if (part === 'part-2')
    {
        let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
        if (inputFile.includes('big'))
        {
            slopes = [];
            for (const r of [ 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 32, 36, 48, 54, 64 ])
            {
                for (const d of [ 1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 47 ])
                {
                    slopes.push([r,d]);
                }
            }
        }

        let totalTrees = 1n;
        for (const [ x, y ] of slopes)
        {
            let cell;
            let currTrees = 0n;
            while ((cell = map.descend(x, y)) !== undefined)
            {
                if (cell === '#') ++currTrees;
            }
            totalTrees *= currTrees === 0n ? 1n : currTrees;
            map.reset();
        }
        console.timeEnd(`Day-3: ${part}`);
        console.log(`Total Trees: ${totalTrees}`);
    }
    else
    {
        console.timeEnd(`Day-3: ${part}`);
        throw Error('must specify part-1 or part-2');
    }
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
