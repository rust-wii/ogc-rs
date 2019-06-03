@echo off

REM SET DEVKITPRO
set DEVKITPRO="C:/devkitPro"
set DEVKITPPC="C:/devkitPro/devkitPPC/powerpc-eabi/include"
set CLANGINCLUDE="C:/Program Files/LLVM/lib/clang/8.0.0/include"

bindgen "%DEVKITPRO%/libogc/include/ogcsys.h" ^
-o src/ogc.rs ^
--rust-target nightly ^
--use-core ^
--distrust-clang-mangling ^
--no-layout-tests ^
--ctypes-prefix "::libc" ^
--no-prepend-enum-name ^
--generate "functions,types,vars" ^
--blacklist-type "u(8|16|32|64)" ^
--blacklist-type "i(8|16|32|64)" ^
--blacklist-type "f(32|64)" ^
-- ^
--target=powerpc-none-eabi ^
--sysroot=%DEVKITPPC%/powerpc-eabi ^
-isystem%DEVKITPPC%/powerpc-eabi/include ^
-isystem%CLANGINCLUDE% ^
-I$DEVKITPRO/libogc/include ^
-mfloat-abi=hard ^
-march=powerpc ^
-nostdinc ^
-Wno-macro-redefined ^
-Wno-incompatible-library-redeclaration ^
-DHW_RVL
