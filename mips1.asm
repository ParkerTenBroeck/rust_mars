
la  $t2, 0xFFFF0000
la  $t1, 0x10040000
loop:

sw $t2, 0($t1)
addi $t1, $t1, 4
j loop