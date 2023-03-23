import "std/stdio.h"
import "std/stdlib.h"
import "std/string.h"
import "std/stdbool.h"
import "std/ctype.h"

external struct FILE end

func read_to_string(path: const char*): const cstring
    var file: FILE* = fopen(path, "r")
    if file == null
        fprintf(stderr, "Could not open file %s\n", path)
        exit(1)
    end

    fseek(file, 0, SEEK_END)
    var size: usize = ftell(file)
    rewind(file)

    var buffer: cstring = malloc(size + 1)

    if buffer == null
        fprintf(stderr, "Could not allocate memory for file %s\n", path)
        exit(1)
    end

    fread(buffer, size, 1, file)
    fclose(file)
    buffer[size] = '\0'
    return buffer
end

enum Type : int
    Int = 0
    String = 1
end

struct Variable
    name: const char*
    var_type: Type
    value: int
end

func main(argc: int, argv: const char**)
    var file_name: const cstring = argv[1] // <--- BIG PROBLEM HERE, CAUSE: generics function calls and indexing clash with each other
    var source: cstring = read_to_string(file_name)
end