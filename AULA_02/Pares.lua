local tabela = {}
for i = 0, 100 do
  tabela[i] = math.random(1, 100)
  print(tabela[i])
end
local par = 0 
for i = 0, 100 do
  if tabela[i] % 2 == 0 then
    par = par + 1 
  end
end
  print("Pares: ", par)