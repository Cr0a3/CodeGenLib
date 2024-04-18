pusha
mov edx, 0 
mov eax, dividend
div divisor_register
mov output_reg, eax
mov output_mod_reg, edx

push output_reg
push output_mod_reg

popa