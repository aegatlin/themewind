# themewindcss

themewindcss is an experiment in generating tailwindcss classes.

the idea is to eventually take a `theme.json` file and generate a `classes.json`
file from it. this would be a script that you could run anywhere and the
`classes.json` file could be handled in a variety of ways depending on your
language and framework of choice.

for example, you could just manually copy and paste each string of classes into
your components as a one-time setup for your components.

for example, you could rename the file to `classes.js` and `export const classes
= ...` and then you read the classes in programmatically in, for example, React,
`<Button className={classes.button} />`.

these classes could one day be controlled within the `theme.json` or some other
configuration so that the output component classes are made for particular
libraries in your language and framework of choice.

for example, headlessui has a bunch of complex components that could have
bespoke component outputs.

i have also built an exploratory npm lib called react-rejects and these are the
direct inspiration behind what are for now the output components.

the output components are crucial to the success of this lib, so in many ways
this lib is inadequate without knowledge of the component form (aka internal
html structure) and, to a lesser extent, function (aka internal js logic).
that said, i suspect this will accomplish the desired outcomes of 80% of the
use cases.