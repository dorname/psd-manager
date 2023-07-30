<script setup>
import Column from "./Column.vue"
defineProps({
    tableData: {
        type: Array,
        required: true
    },
    columns: {
        type: Array,
        required: false
    }
});
</script>
<template>
    <el-container>
        <el-main>
            <el-table :data="tableData" style="width: 100%">
                <Column v-for="(column, index) in columns" :key="index" :propertys ="column" />
            </el-table>
        </el-main>
    </el-container>
</template>
