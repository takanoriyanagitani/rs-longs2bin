iwasi="./target/wasm32-wasip1/release-wasi/longs2bin_std.wasm"

wasm-opt \
	-Oz \
	-o opt.wasm \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	--enable-simd \
	"${iwasi}"
