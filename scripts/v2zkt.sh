#!/usr/bin/bash
cat > synth.tmp.ys <<EOF
read -sv $1 
synth -flatten -top $2
abc -g AND,NAND,OR,NOR,XOR,XNOR
opt_clean -purge
write_json $2.tmpsynth.json
EOF

yosys synth.tmp.ys
rm synth.tmp.ys

python3 ./scripts/yjs2zkt.py $2.tmpsynth.json $2 ${3:-$2.zkt}
rm $2.tmpsynth.json
