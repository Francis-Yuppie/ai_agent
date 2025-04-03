<script setup>
import { ProductService } from '@/service/ProductService';
import { onMounted, ref } from 'vue';

const products = ref(null);


onMounted(() => {
    ProductService.getProductsSmall().then((data) => (products.value = data));
});
</script>

<template>
    <div class="card">
        <div class="font-semibold text-xl mb-4">Recent Scans</div>
        <DataTable :value="products" :rows="5" :paginator="true" responsiveLayout="scroll">
            <Column style="width: 15%" header="Canister ID">
                <template #body="slotProps">
                    <!-- <img :src="`https://primefaces.org/cdn/primevue/images/product/${slotProps.data.image}`" :alt="slotProps.data.image" width="50" class="shadow" /> -->
                     {{ slotProps.data.code }}
                </template>
            </Column>
            <Column field="name" header="Canister " style="width: 25%"></Column>
            <Column field="price" header="Score" style="width: 20%">
                <template #body="slotProps">
                    {{ slotProps.data.price  }}%
                </template>
            </Column>
            <!-- //vulnerbility -->
             <column field="category" header="Vulnerbility"></column>

            <Column style="width: 15%" header="View">
                <template #body>
                    <Button icon="pi pi-eye" type="button" class="p-button-text"></Button>
                </template>
            </Column>
        </DataTable>
    </div>
</template>
