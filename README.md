<div class="oranda-hide">

# Hexagon

</div>

Hexagon is a programming language for Hex Casting. Hexagon is a superset of the [hexpattern](https://github.com/object-Object/vscode-hex-casting) format, adding variables, if statements, and more. 

[![release](https://github.com/Master-Bw3/Hexagon/actions/workflows/release.yml/badge.svg)](https://github.com/axodotdev/oranda/actions/workflows/release.yml)
[![web](https://github.com/Master-Bw3/Hexagon/actions/workflows/web.yml/badge.svg?branch=master)](https://github.com/axodotdev/oranda/actions/workflows/web.yml)


<div class="oranda-hide">

## Installing
Download the latest release [here](https://github.com/Master-Bw3/Hexagon/releases/latest)
</div>

## Usage
Interpret a file
```
hexagon run filename.hexagon
```
Compile a file
```
hexagon build filename.hexagon
```
Send a hex to [Hex Server](https://github.com/Master-Bw3/hex_server)
```
hexagon send http://localhost:9000/hexPost filename.hexagon
```

## Syntax

Actions are written in the hexpattern format

```
Mind's Reflection
Compass Purification
```

Actions that retrieve information from the world must have a value set for the interpreter (not needed for compiler)

```
Scout's Distillation: Null
Zone Distillation: Player: [@Caster]
```

Numerical Reflection and Bookkeepers also require a value to be set
```
Numerical Reflection: 1
Bookkeeper's Gambit: v-vv-
```

### Iota Syntax
Iotas are written in the following format:
```
1                              //Number
(1,2,3)                        //Vector
[1, (1, 2, 3)]                 //List
@Caster                        //Entity 
Null                           //Null
Garbage                        //Garbage
True                           //Bool
NORTHEAST qaq                  //Pattern via signature
Numerical Reflection: 1        //Pattern via name
"hello world"                  //String
[(3, 2) | 1, 1; 2, 2; 3, 3]    //Matrix
[(0, 0)]                    //Empty Matrix
```

### Embedding Iotas
```
<Iota>: direct insertion, no escape
<{Iota}>: embed with intro/retro/flock
<\Iota>: embed with consideration(s)
<<Iota>>: embed with intro or considerations, whichever is shorter
```

### Variables
Note: currently, all variables are global and are never deallocated.
```
Mind's Reflection
Compass Purification 
Store($pos)               //Remove top iota from stack and store in $pos

Mind's Reflection
Compass Purification 
Copy($pos)                //Copy top iota on stack and store in $pos

$pos                      //Push pos to the stack
Place Block
```

### If / Else
```
<\True>
Store($condition)
...
if {$condition} then {
	...
} else {
	...
}

//else branch is optional
if {$condition} then {
	...
}

//else if
if {$condition1} then {
	...
} else if ($condition2) then {
	...
} else {
	...
}
```

If statements will evaluate the condition and use the top Iota on the stack to determine which branch to take. Note that this does not isolate the stack, so the code below is perfectly valid: 
```
<\True>
if {} then {
	//this branch will be pushed to the stack
	...
} else {
	...
}
```

The `then` and `else` blocks will be pushed to the stack. To evaluate the block, use Hermes' Gambit or its ilk.
```
if {<\True>} then {
	...
} else {
	...
}
Hermes' Gambit //evaluates the 'then' branch


```

### Macros
Macros are defined using this syntax:
```
#define Macro Name (DIRECTION signature) ... {
...
}
```
As an example:
```
#define Duplicate Thrice (SOUTH_EAST edd) = num, num -> num {
	Numerical Reflection: 3
	Gemini Gambit
}
```

Note: everything between the signature and first curly bracket is ignored, so the following is also valid:
```
#define Duplicate Thrice (SOUTH_EAST edd)
{
	Numerical Reflection: 3
	Gemini Gambit
}
```

When macros are used in a hex, they get expanded, not evaluated.
```
Mind's Reflection
Duplicate Thrice     //expand a macro

{
	Duplicate Thrice //expand a macro into a list
}




<\Duplicate Thrice>  //embed the pattern associated with the macro

<Duplicate Thrice>   //will cause a mishap
```

## Config
By default, Hexagon looks for a `config.toml` file in the current directory. A different file can also be specified:
```
hexagon run example.hexagon example.toml
```

### Config Syntax 
```
//register an akashic library
[[libraries]]
location = [0, 0, 0]
"adda" = "@Zombie"
"qaaq" = "5"

//register another library
[[libraries]]
location = [1, 2, 3]

//register an entity
[[entities]]
name = "Zombie"
uuid = "[I;542246046,714361767,-2088965103,2106423580]" //optional, defaults to 0s
type = "Monster"
item = "Focus" //optional, defaults to None
iota = "1"     //optional, defaults to None
```
