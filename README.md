<p>This code allows you to create a png using a list of instructions placed in a "cmds.txt" file which will produce a "output.png."</p>

<p>Each line of the "cmds.txt" must contain 3 parts</p>

* action
* equation
* RGB

There are 3 actions:
* add
* subtract
* set

<p>An equation could be anything that contains the variable x. (excluding abs(), sqrt(), and other math functions)</p>
<p>For now as it's a prototype all negative values must be represented as (0-a).</p>

<p>The RGB part must be three values from 0->255.</p>
<p>The lines should look like 'action, equation, RGB'</p>
<p>The commas are mandatory or it will not run. Whitespace should not matter as the command is trimmed, but for your sake I recommend keeping it consistent.</p>

# Example
## cmds.txt
* add, 0, 255 0 0
* add, 5, 0 255 0
* add, x, 0 0 255
* set, (0-1)*(x)+99, 255 0 255
* subtract, (x-10)^2, 200 200 200

## Output

![output](https://github.com/KingOws/image-editor/assets/79430103/fcb815ea-1f83-4e41-9075-fb04a1b18716)


# Running the Code
You will most likely have to use the commands 'cargo build' and then 'cargo run' as this is only the Rust Code and not an executable.

Sorry for bad and unorganized code. It's kinda all over the place since I used ChatGPT, and this is just for fun anyways.
