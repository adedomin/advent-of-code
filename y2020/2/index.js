#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');
const policyParser = /^(?<min>\d+)-(?<max>\d+) (?<chr>.): (?<pass>.*)$/;

async function main([ part, inputFile ])
{
    const input = (await readInput(inputFile))
        .split('\n')
        .map(p => p.match(policyParser).groups);

    console.time(`Day-2: ${part}`);
    if (part === 'part-1')
    {
        let totalValid = 0;
        for (const policy of input)
        {
            let chrCount = 0;
            for (const chr of policy.pass)
            {
                if (chr === policy.chr) ++chrCount;
            }
            if (+policy.min <= chrCount && +policy.max >= chrCount)
            {
                totalValid += 1;
            }
        }
        console.timeEnd(`Day-2: ${part}`);
        console.log('Valid Passwords: ', totalValid);
    }
    else if (part === 'part-2')
    {
        let totalValid = 0;
        for (const policy of input)
        {
            if ((policy.chr === policy.pass[policy.min-1]) ^
                (policy.chr === policy.pass[policy.max-1]))
            {
                ++totalValid;
            }
        }
        console.timeEnd(`Day-2: ${part}`);
        console.log('Valid Passwords: ', totalValid);
    }
    else
    {
        console.timeEnd(`Day-2: ${part}`);
        throw Error('must specify part-1 or part-2');
    }
}

main(argv.slice(2)).catch(err =>
{
    console.error(err);
    exit(1);
});
