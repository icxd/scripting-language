import "std/stdio.h"
import "std/stdlib.h"
import "std/string.h"
import "std/stdbool.h"
import "std/ctype.h"

enum Statement
    Variable(name: const char*, value: int)
    Sample(x: int)
end

func test()
    var x: Statement = Statement.Variable(name: "test", value: 5)
    var y: Statement = Statement.Sample(x: 5)

    printf("%s = %d\n", x.name, x.value)
    printf("Sample %d\n", y.x)
end

func main(argc: int, argv: const char**): int
    if argc < 2
        printf("Usage: %s <filename>\n", argv[0])
        return 1
    end

    test()
    
    return 0
end