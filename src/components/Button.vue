<script setup>
defineProps({
  text:{
    type: String,
    required: true
  }
})

</script>
<template>
    <button id="home" type="button">{{ text }}</button>
    <!-- <el-button>{{text}}</el-button> -->
</template>