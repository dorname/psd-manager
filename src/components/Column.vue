<script setup>
import { h } from 'vue';
const props = defineProps({
    propertys: {
        type: Object,
        required: true
    }
});
const style = props.propertys.style;
console.log(style);
// const headerRender = ({ column, $index }) => {
//     return h('span', `自定义表头：${column.label}`);
// }
</script>
<template>
    <!-- <el-table-column v-bind="propertys" label-class-name = "test" :render-header = "headerRender"> -->
    <el-table-column v-bind="propertys">
        <template #header="{ column }">
            <span class="column-header" :style="style" >自定义表头：{{ column.label }}</span>
        </template>
    </el-table-column>
</template>
<style scoped>
/**
* 列组件样式调整定义
*/
/* :deep(.el-table .test) {
    color:black !important;
    background-color: blue !important;
 }  */
 /* .column-header {
    color:black !important;
 } */
</style>