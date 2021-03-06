pushd ../blarggs-tests/cpu_instrs/source
./clean.sh

# ./build.sh "03-op sp,hl" # index out of bounds: the len is 8192 but the index is 8192
# ./build.sh "07-jr,jp,call,ret,rst" # Attempts to call 0xFC
# ./build.sh "08-misc instrs" # never completes


# ./build.sh "01-special" # PASSING
# ./build.sh "02-interrupts" PASSING
# ./build.sh "04-op r,imm" # PASSING
# ./build.sh "05-op rp" # PASSING
# ./build.sh "06-ld r,r" # PASSING
# ./build.sh "09-op r,r" # PASSING
# ./build.sh "10-bit ops" # PASSING
# ./build.sh "11-op a,(hl)" # PASSING

popd

cargo run -- \
  --boot_rom ../gb_test_roms/DMG_ROM.bin \
  --game_rom ../blarggs-tests/cpu_instrs/source/test.gb \
  --skip_boot \
  $1
