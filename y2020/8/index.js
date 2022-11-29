#!/usr/bin/env node
'use strict';

const { exit, argv } = require('process');
const readInput = require('../lib/read-input.js');

class GameConsole
{
    constructor(input)
    {
        /** @type {Array<[string, number, boolean]>} */
        this.input = input;
        this.pc = 0;
        this.oldPc = 0;
        this.accumulate = 0;
    }

    isCycle()
    {
        return this.input[this.pc][2];
    }

    nop()
    {
        return ++this.pc;
    }

    acc(arg)
    {
        this.accumulate += arg;
        return ++this.pc;
    }

    jmp(arg)
    {
        this.pc += arg;
        return this.pc;
    }

    toggleNopJmp(pc)
    {
        if (this.input[pc][0] === 'jmp')
        {
            this.input[pc][0] = 'nop';
        }
        else if (this.input[pc][0] === 'nop')
        {
            this.input[pc][0] = 'jmp';
        }
        else
        {
            throw Error(`Not nop or jmp at ${pc}`);
        }
    }

    isDone()
    {
        return this.pc >= this.input.length || this.pc < 0;
    }

    reset()
    {
        for (let i = 0; i < this.input.length; ++i) this.input[i][2] = false;
        this.pc = 0;
        this.accumulate = 0;
    }

    step()
    {
        const currentInst = this.input[this.pc];
        currentInst[2] = true;
        switch (currentInst[0])
        {
        case 'nop':
            this.nop();
            break;
        case 'acc':
            this.acc(currentInst[1]);
            break;
        case 'jmp':
            this.jmp(currentInst[1]);
            break;
        default:
            throw Error('invalid instruction');
        }
        return this.pc < this.input.length && this.pc > -1;
    }
}

/**
 * Main solution
 * @params {string[]}            argv
 * @params {('part-1'|'part-2')} argv.0 - part-1 or part-2.
 * @params {(string|undefined)}  argv.1 - inputfile or stdin (undefined).
 * @throws {Error}
 */
async function main([ part, inputFile ])
{
    /** @type {Array<[string, number, boolean]>} */
    const input = (await readInput(inputFile))
        .split('\n')
        .map(i =>
        {
            const [ inst, arg ] = i.trim().split(' ');
            return [ inst, +arg, false ];
        });

    console.time(`Day 8: ${part}`);
    const machine = new GameConsole(input);
    if (part === 'part-1')
    {
        while (!machine.isCycle()) machine.step();
        console.timeEnd(`Day 8: ${part}`);
        console.log(`Accumulator at first cycle: ${machine.accumulate}`);
    }
    else /* FIXME: this answer is basically brute forcing */
    {
        while (!machine.isCycle()) machine.step();
        let lastJmp = machine.pc;
        // find the latest instruction before we cycled
        for (; machine.input[lastJmp][2]; ++lastJmp);
        for (let i = 0; i < lastJmp;  ++i)
        {
            if (machine.input[i][0] === 'nop' || machine.input[i][0] === 'jmp')
            {
                machine.reset();
                machine.toggleNopJmp(i);
                while (machine.step() && !machine.isCycle());
                if (machine.isDone())
                {
                    console.timeEnd(`Day 8: ${part}`);
                    console.log(`Accumulator at end: ${machine.accumulate}, replaced: ${i}`);
                    break;
                }
                else
                {
                    machine.toggleNopJmp(i);
                }
            }
        }
    }
}

main(argv.slice(2)).catch(e =>
{
    console.error(e);
    exit(1);
});
