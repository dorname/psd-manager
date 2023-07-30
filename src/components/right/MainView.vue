<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
// invoke('genpsd', { seed: 'test',length:16 })
//   // `invoke` 返回的是一个 Promise
//   .then((response) => console.log(response))
defineProps({
  msg: String,
})

const count = ref(0)
</script>
<template> 

</template>