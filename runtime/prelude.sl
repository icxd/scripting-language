import "std/stdio.h"
import "std/stdlib.h"
import "std/string.h"

struct String
    data: char*
    len: int

    constructor: func(char*): String*
    
    c_str: func(String*): char*
    length: func(String*): int
end

func String.c_str(self: String*): char* => self.data
func String.length(self: String*): int => self.len
func String.constructor(data: char*): String*
    var self: String* = malloc(sizeof String*)
    self.data = data
    self.len = strlen(data)
    self.constructor = String.constructor
    self.c_str = String.c_str
    self.length = String.length
    return self
end

struct Matrix
    data: int*
    rows: int
    cols: int

    constructor: func(int*, int, int): Matrix*
    get: func(Matrix*, int, int): int
    set: func(Matrix*, int, int, int)
    print: func(Matrix*)
end

func Matrix.get(self: Matrix*, row: int, col: int): int => self.data[row * self.cols + col]
func Matrix.set(self: Matrix*, row: int, col: int, value: int)
    self.data[row * self.cols + col] = value
end
func Matrix.print(self: Matrix*)
    var i: int = 0
    var j: int = 0

    while i < self.rows
        while j < self.cols
            printf("%d ", self.get(i, j))
            j = j + 1
        end
        printf("\n")
        j = 0
        i = i + 1
    end
end
func Matrix.constructor(data: int*, rows: int, cols: int): Matrix*
    var self: Matrix* = malloc(sizeof Matrix*)
    self.data = data
    self.rows = rows
    self.cols = cols
    self.constructor = Matrix.constructor
    self.get = Matrix.get
    self.set = Matrix.set
    self.print = Matrix.print
    return self
end

struct Array
    data: int*
    len: int

    constructor: func(int*, int): Array*
    get: func(Array*, int): int
    set: func(Array*, int, int)
    print: func(Array*)
end

func Array.get(self: Array*, index: int): int => self.data[index]
func Array.set(self: Array*, index: int, value: int)
    self.data[index] = value
end
func Array.print(self: Array*)
    var i: int = 0
    while i < self.len
        printf("%d ", self.get(i))
        i = i + 1
    end
    printf("\n")
end
func Array.constructor(data: int*, len: int): Array*
    var self: Array* = malloc(sizeof Array*)
    self.data = data
    self.len = len
    self.constructor = Array.constructor
    self.get = Array.get
    self.set = Array.set
    self.print = Array.print
    return self
end

func main(argc: int, argv: const cstring*)
    var array: Array* = new Array(malloc(sizeof int* * 9), 9)
    var i: int = 0
    while i < 9
        array.set(i, i + 1)
        i = i + 1
    end
    array.print()
    var matrix: Matrix* = new Matrix(malloc(sizeof int* * 9), 3, 3)
    i = 0
    while i < 9
        matrix.set(i / 3, i % 3, i + 1)
        i = i + 1
    end
    matrix.print()
end