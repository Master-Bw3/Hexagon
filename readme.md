# Hexagon
Hexagon is a programming language for Hex Casting. Hexagon is a superset of the [hexpattern](https://github.com/object-Object/vscode-hex-casting) format, adding variables, if statements, and more. 

## Installing
Either download the latest release file or install via Cargo:
```cargo install --git https://github.com/Master-Bw3/Hexagon```

## Usage
Interpret a file
```
hexagon run filename.hexcasting
```
Compile a file
```
hexagon build filename.hexcasting
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

## Config
By default, Hexagon looks for a `config.toml` file in the current directory. A different file can also be specified:
```
hexagon run example.hexcasting example.toml
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
type = "Monster"
item = "Focus" //optional
iota = "1"     //optional
```
