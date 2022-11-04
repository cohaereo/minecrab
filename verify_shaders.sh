for i in src/shaders/*.spv
do
echo $i
RUST_LOG=error naga $i
done
