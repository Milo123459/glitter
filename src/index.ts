import chalk from 'chalk';
import meow from 'meow';
import terminalLink from 'terminal-link';
import { execSync } from 'child_process';
import fs from 'fs/promises';

const cli = meow(`
See the ${terminalLink('readme', 'https://github.com/Milo123459/glitter')} for information.
`, {
    flags: {
        branch: {
            type: 'string',
            isRequired: false,
            default: execSync('git rev-parse --abbrev-ref HEAD').toString().split(/\n/gi)[0],
            alias: 'b'
        },
        commit: {
            type: 'string',
            isRequired: true,
            alias: 'c'
        },
        args: {
            type: 'string',
            isRequired: false,
            alias: 'a'
        }
    }
})

console.log(cli.flags);