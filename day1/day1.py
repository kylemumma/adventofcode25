def rotate(start: int, rotation: str) -> tuple[int,int]:
	delta = int(rotation[1:])
	if rotation[0] == "L":
		delta *= -1
	rotated = start + delta
	passes = 0
	if rotated == 0:
		passes = 1
	elif rotated > 99:
		passes = int(rotated/100)
	elif rotated < 0:
		passes = 1+(-1*int(rotated/100))
		if start == 0:
			passes -= 1
	
	return (rotated%100, passes)

with open("input.txt", 'r') as f:
	ctr = 0
	curr = 50
	for line in f:
		n,p = rotate(curr, line.strip())
		curr = n
		ctr+=p
	print(ctr)
