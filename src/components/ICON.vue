<!-- 
  自定义element-plus图标组件
  图标名称：iconName
  标签类名：userCls
  图标颜色: color
  图标大小: size
  样式大小: style
  点击事件监听器：handler
 -->
<script setup>
const props = defineProps({
  iconName:{
    type: String,
    required: true
  },
  userCls:{
    type:String,
    required: false
  },
  color:{
    type:String,
    required:false
  },
  size:{
    type:String,
    required:false
  },
  style:{
    type:Object,
    required:false
  },
  handler:{
    type:Function,
    required:false
  }
})

const icon = props.iconName;
const color = props.color;
const size = props.size;
const userCls = "userCls "+props.userCls;
const iconStyle = props.style
</script>

<template>
  <div :class="userCls" :style="iconStyle">
    <el-icon :color="color" :size="size" @click="props.handler"> <component :is="icon"></component></el-icon>
  </div>
</template>

<style scoped>
/*
为图标增加悬浮小手 
 */
.userCls{
  cursor: pointer;
}
</style>

