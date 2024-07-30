# md2src

Generates source code from markdown files.

## Example

hello-world.md:
````markdown
# Hello World

This is a hello-world example:
```c
printf("Hello World\n");
```
````

Run `md2src --lang c --include stdio.h hello-world.md`:

```c
#include <stdio.h>

////////////////////////////////////////////////////////////////////////////////
// # Hello World
// 
// This is a hello-world example:
static void line_4(void) {
	printf("Hello World\n");
}
////////////////////////////////////////////////////////////////////////////////
int main(int argc, char *argv[]) {
	line_4();
	return 0;
}
```

## Supported Language

* c
* shell script
