// Wrapper file to re-export stuff as C symbols

#include "c/src/MurmurHash2.h"
#include "murmur2.h"

extern "C" {

uint32_t cMurmurHash2        ( const void * key, int len, uint32_t seed ) { return MurmurHash2        (key, len, seed ); }
uint64_t cMurmurHash64A      ( const void * key, int len, uint64_t seed ) { return MurmurHash64A      (key, len, seed ); }
uint64_t cMurmurHash64B      ( const void * key, int len, uint64_t seed ) { return MurmurHash64B      (key, len, seed ); }
uint32_t cMurmurHash2A       ( const void * key, int len, uint32_t seed ) { return MurmurHash2A       (key, len, seed ); }
uint32_t cMurmurHashNeutral2 ( const void * key, int len, uint32_t seed ) { return MurmurHashNeutral2 (key, len, seed ); }
uint32_t cMurmurHashAligned2 ( const void * key, int len, uint32_t seed ) { return MurmurHashAligned2 (key, len, seed ); }

}
