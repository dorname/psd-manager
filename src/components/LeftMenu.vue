<script setup>
import Icon from "./ICON.vue"
const props  = defineProps({
  menus: {
    type: Array,
    required: false
  },
  style:{
    type:Object,
    required:false
  }
})
// const text = props.textMenu;
const menus = props.menus;
const style = props.style;
</script>
<template>
    <div class="menus">
      <span class="menuspan" v-for="menu in menus" :style="style">
      <Icon :icon-name="menu.name" :color="menu.color" :size="menu.size" :handler="menu.handler" />
      </span>
    </div>    
</template>

<style scoped>
.menus {
  background-color: rgb(32, 24, 24);
  display: flex;
  flex-direction: column;
}
</style>