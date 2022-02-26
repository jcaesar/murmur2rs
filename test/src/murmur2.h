#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t cMurmurHash2        ( const void * key, int len, uint32_t seed );
uint64_t cMurmurHash64A      ( const void * key, int len, uint64_t seed );
uint64_t cMurmurHash64B      ( const void * key, int len, uint64_t seed );
uint32_t cMurmurHash2A       ( const void * key, int len, uint32_t seed );
uint32_t cMurmurHashNeutral2 ( const void * key, int len, uint32_t seed );
uint32_t cMurmurHashAligned2 ( const void * key, int len, uint32_t seed );

#ifdef __cplusplus
}
#endif
