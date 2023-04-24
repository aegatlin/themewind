# themewind

experiemental!

themewind generates tailwindcss classes for components based on a theme file.

## Installation

You can `cargo install --git ...` if you want to experiment with it.

One day I would like to distribute it via a crate and a npm package, and maybe
more langs & frameworks as the desire arises. For example, I'd like to get this
working usefully for `.heex` files in elixir.

## Generate classes

Create a `src/themewind` folder (more locations will be supported one day).
Create a `src/themewind/theme.json`.  Run `themewind` from root. It creates
classes in `src/themewind/classes.json`.  You can run `themewind ts` to create
`src/themewind/classes.ts` instead.  Alsos, `themewind js`. More filetypes will
be supported one day.

## Use classes

Once you have your classes, you can manually copy & paste them into desired
components.  Alternatively, if you've ran something like `themewind ts` you can
import the classes directly into a component. For example, in react-typescript:

```tsx import { classes } from "[path-to]/src/themewind/classes"; import {
Button } from "[local-or-lib]";

function Page() { return ( <div> <Button classes={classes.button} /> </div> ); }
```

Best practice tailwindcss is probably to copy and paste the classes string into
the component.  Failure to do so could lead to managing your css in a file
separated from your components, which is a lot like the old way of managing css.
This is a risk to be aware of. My recommendation is to import the classes file
so long as you still feel like you are tweaking your theme, and you can then
benefit from bulk component theme management. At some point, your components
should mature to the point where you are best served having them live _with_
their components, at which point themewind can fade away. 

Another good use case is if you are using a headless component library, and so
you do not control the implementation.  You can still colocate a local "wrapper
component" around the lib's component and your usage of it, and include the
classes there.

## Aspirations

I am making a headless component library. I'd like to gen classes for that.

headlessui by tailwindcss is also a headless component library. I'd like to
gen classes for that. It could output namespaces components, so
`classes.headlessui.popover = {button, panel}`, and
`classes.reactIchabod.popover = {button, panel}`. You could then pick and choose
which supported headless ui lib to apply classes to. Ideally, these would be similar
enough, though, that it wouldn't be necessary, but implementation will likely affect
which sub-components can be styled.

## Warnings 

This is experimental

the output components are crucial to the success of this lib, so in many ways
this lib is inadequate without knowledge of the component form (aka internal
html structure) and, to a lesser extent, function (aka internal js logic).  that
said, i suspect this will accomplish the desired outcomes of 80% of the use
cases.
