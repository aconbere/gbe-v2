pushd ../blarggs-tests/cpu_instrs/source
./build.sh "01-special"
# ./build.sh "02-interrupts"
# ./build.sh "03-op sp,hl"
# ./build.sh "04-op r,imm"
# ./build.sh "05-op rp"
# ./build.sh "06-ld r,r"
# ./build.sh "07-jr,jp,call,ret,rst"
# ./build.sh "08-misc instrs"
# ./build.sh "09-op r,r"
# ./build.sh "10-bit ops"
# ./build.sh "11-op a,(hl)"

popd

cargo run -- \
  --boot_rom ../gb_test_roms/DMG_ROM.bin \
  --game_rom ../blarggs-tests/cpu_instrs/source/test.gb
