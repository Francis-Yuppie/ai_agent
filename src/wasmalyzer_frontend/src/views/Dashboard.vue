<script setup>
import StatsWidget from '@/components/dashboard/StatsWidget.vue';
import RecentSalesWidget from '@/components/dashboard/RecentSalesWidget.vue';
import CanisterHealth from  '@/components/dashboard/BestSellingWidget.vue'
import PieChart from '@/components/dashboard/PieChart.vue';
</script>

<template>
    <div class="grid grid-cols-12 gap-8">
        <StatsWidget />
    </div>
    <div>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-5 mb-5">
        <PieChart />
        <CanisterHealth />
    </div>
    <RecentSalesWidget />
    </div>
</template>
