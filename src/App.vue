<script setup>
// import LeftMenu from './components/LeftMenu.vue'
import RightMain from './components/RightMain.vue'
// const rawHtml = '<span style="color: red">This should be red.</span>'
// const menus = [{
//   name: "House",
//   handler:function(){
//     alert("Home<<<<<");
//   },
//   size:"25",
//   color:"white",
//   style:{
//     height:25
//   }
// }, {
//   name: "Search",
//   size:"25",
//   color:"white",
//   style:{
//     height:50
//   }
// }];
</script>

<template>
  <body class="mainBody">
    <!-- <LeftMenu :menus="menus" :style="myStyle" /> -->
    <!-- <LeftMenu :menus="menus" /> -->
    <RightMain />
    <!-- <p><span v-html="rawHtml"></span></p> -->
  </body>
</template>

<style scoped>
body .mainBody {
  display: flex;
}
</style>
