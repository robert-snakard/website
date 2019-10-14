'use strict';

const Metalsmith = require('metalsmith'),
      layouts    = require('metalsmith-layouts'),
      inplace    = require('metalsmith-in-place');

const toUpper = function (string) {
    "use strict";
    return string.toUpperCase();
}

const templateConfig = {
    engineOptions: {
        filters: {
            toUpper: toUpper
        }
    }
}

Metalsmith(__dirname)
    .clean(true)
    .source('./src')
    .destination('./build')
    .use(layouts(templateConfig))
    .use(inplace(templateConfig))
    .build(function (err) { 
        if(err) 
            console.log(err) 
    });
