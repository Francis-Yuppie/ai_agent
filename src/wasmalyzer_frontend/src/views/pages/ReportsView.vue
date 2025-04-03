<script setup>
import { CanisterService } from '@/service/CanisterService';
import { FilterMatchMode, FilterOperator } from '@primevue/core/api';
import { onBeforeMount, ref } from 'vue';

const reports = ref(null);
const filters = ref(null);
const loading = ref(true);
const expandedRows = ref([]);

// Get status severity class
function getSeverity(status) {
    switch (status) {
        case 'RUNNING':
            return 'success';
        case 'STOPPED':
            return 'warn';
        case 'FAILED':
            return 'danger';
        default:
            return null;
    }
}

// Get vulnerability severity class
function getVulnerabilitySeverity(severity) {
    switch (severity) {
        case 'Critical':
        case 'High':
            return 'danger';
        case 'Medium':
            return 'warn';
        case 'Low':
            return 'info';
        case 'Informational':
            return 'secondary';
        default:
            return null;
    }
}

onBeforeMount(() => {
    // Load canister scan reports
    CanisterService.getCanisterReports().then((data) => {
        reports.value = data;
        loading.value = false;
        // Format dates
        reports.value.forEach((report) => {
            report.scanDate = new Date(report.scanDate);
            report.vulnerabilities.forEach((vuln) => {
                vuln.detectedDate = new Date(vuln.detectedDate);
            });
        });
    });

    initFilters();
});

function initFilters() {
    filters.value = {
        global: { value: null, matchMode: FilterMatchMode.CONTAINS },
        name: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }] },
        canisterId: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }] },
        type: { value: null, matchMode: FilterMatchMode.IN },
        scanDate: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.DATE_IS }] },
        status: { operator: FilterOperator.OR, constraints: [{ value: null, matchMode: FilterMatchMode.EQUALS }] },
        vulnerabilityCount: { value: [0, 20], matchMode: FilterMatchMode.BETWEEN }
    };
}

function formatDate(value) {
    return value.toLocaleDateString('en-US', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
    });
}

function formatCycles(value) {
    if (value) return value.toLocaleString('en-US');
    return '';
}

// Function to expand all rows
function expandAll() {
    expandedRows.value = reports.value.filter(p => p.vulnerabilities && p.vulnerabilities.length > 0);
}

// Function to collapse all rows
function collapseAll() {
    expandedRows.value = [];
}

// Define mock service - this would need to be implemented
// in a separate file in a real application
CanisterService.getCanisterReports = () => {
    return Promise.resolve([
        {
            id: '1001',
            canisterId: 'rrkah-fqaaa-aaaaa-aaaaq-cai',
            name: 'Internet Identity',
            icon: 'canister-placeholder.svg',
            scanDate: '2025-03-28T10:15:30Z',
            type: 'Backend Service',
            status: 'RUNNING',
            cyclesUsed: 450000,
            scanDuration: '10.543s',
            vulnerabilityCount: 0,
            vulnerabilities: []
        },
        {
            id: '1002',
            canisterId: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
            name: 'NNS Dapp',
            icon: 'canister-placeholder.svg',
            scanDate: '2025-03-29T14:22:10Z',
            type: 'Smart Contract',
            status: 'STOPPED',
            cyclesUsed: 780000,
            scanDuration: '9.212s',
            vulnerabilityCount: 3,
            vulnerabilities: [
                {
                    id: 'V001',
                    type: 'Buffer Overflow',
                    severity: 'High',
                    location: 'src/main.mo:127',
                    detectedDate: '2025-03-29T14:22:10Z',
                    description: 'Potential buffer overflow vulnerability detected when processing large inputs.',
                    recommendation: 'Add proper bounds checking for input arrays and implement size validation.'
                },
                {
                    id: 'V002',
                    type: 'Integer Overflow',
                    severity: 'Medium',
                    location: 'src/lib/math.mo:56',
                    detectedDate: '2025-03-29T14:22:10Z',
                    description: 'Possible integer overflow when calculating large balances.',
                    recommendation: 'Use safe math operations or check bounds before arithmetic operations.'
                },
                {
                    id: 'V003',
                    type: 'Uninitialized Memory',
                    severity: 'Medium',
                    location: 'src/modules/accounts.mo:89',
                    detectedDate: '2025-03-29T14:22:10Z',
                    description: 'Potential use of uninitialized memory in account creation.',
                    recommendation: 'Ensure all variables are properly initialized before use.'
                }
            ]
        },
        {
            id: '1003',
            canisterId: 'oeee4-qaaaa-aaaak-qaaeq-cai',
            name: 'ICRC-1 Ledger',
            icon: 'canister-placeholder.svg',
            scanDate: '2025-03-30T09:45:22Z',
            type: 'Smart Contract',
            status: 'FAILED',
            cyclesUsed: 920000,
            scanDuration: '11.876s',
            vulnerabilityCount: 4,
            vulnerabilities: [
                {
                    id: 'V004',
                    type: 'Race Condition',
                    severity: 'Critical',
                    location: 'src/ledger.mo:213',
                    detectedDate: '2025-03-30T09:45:22Z',
                    description: 'Critical race condition detected in transaction processing.',
                    recommendation: 'Implement proper synchronization mechanisms for concurrent transactions.'
                },
                {
                    id: 'V005',
                    type: 'Memory Leak',
                    severity: 'High',
                    location: 'src/storage/stable.mo:78',
                    detectedDate: '2025-03-30T09:45:22Z',
                    description: 'Memory leak detected in stable storage handling.',
                    recommendation: 'Properly release resources after use and implement garbage collection.'
                },
                {
                    id: 'V006',
                    type: 'Null Pointer Dereference',
                    severity: 'Medium',
                    location: 'src/utils/validation.mo:42',
                    detectedDate: '2025-03-30T09:45:22Z',
                    description: 'Possible null pointer dereference when validating transaction data.',
                    recommendation: 'Add null checks before accessing optional values.'
                },
                {
                    id: 'V007',
                    type: 'Use After Free',
                    severity: 'High',
                    location: 'src/memory/heap.mo:156',
                    detectedDate: '2025-03-30T09:45:22Z',
                    description: 'Potential use after free vulnerability in memory management.',
                    recommendation: 'Ensure memory is not accessed after being freed or reassigned.'
                }
            ]
        },
        {
            id: '1004',
            canisterId: '5cf74-raaa-aaaae-qaaba-cai',
            name: 'Asset Storage',
            icon: 'canister-placeholder.svg',
            scanDate: '2025-04-01T16:30:05Z',
            type: 'Asset Storage',
            status: 'STOPPED',
            cyclesUsed: 560000,
            scanDuration: '8.321s',
            vulnerabilityCount: 2,
            vulnerabilities: [
                {
                    id: 'V008',
                    type: 'Infinite Loop',
                    severity: 'Low',
                    location: 'src/asset/chunking.mo:67',
                    detectedDate: '2025-04-01T16:30:05Z',
                    description: 'Potential infinite loop when processing malformed asset chunks.',
                    recommendation: 'Add a maximum iteration count or timeout condition to loop structures.'
                },
                {
                    id: 'V009',
                    type: 'Stack Overflow',
                    severity: 'Medium',
                    location: 'src/recursive/directory.mo:124',
                    detectedDate: '2025-04-01T16:30:05Z',
                    description: 'Possible stack overflow with deeply nested directory structures.',
                    recommendation: 'Implement iterative approach instead of recursion or limit recursion depth.'
                }
            ]
        },
        {
            id: '1005',
            canisterId: 'br5f7-7uaaa-aaaaa-qaaca-cai',
            name: 'Governance DB',
            icon: 'canister-placeholder.svg',
            scanDate: '2025-04-02T11:20:15Z',
            type: 'Database',
            status: 'FAILED',
            cyclesUsed: 680000,
            scanDuration: '9.895s',
            vulnerabilityCount: 5,
            vulnerabilities: [
                {
                    id: 'V010',
                    type: 'Buffer Overflow',
                    severity: 'Critical',
                    location: 'src/db/query.mo:312',
                    detectedDate: '2025-04-02T11:20:15Z',
                    description: 'Critical buffer overflow in database query processing.',
                    recommendation: 'Implement proper bounds checking and input validation.'
                },
                {
                    id: 'V011',
                    type: 'Heap Corruption',
                    severity: 'Critical',
                    location: 'src/memory/allocator.mo:89',
                    detectedDate: '2025-04-02T11:20:15Z',
                    description: 'Potential heap corruption in memory allocator.',
                    recommendation: 'Review memory management implementation and add integrity checks.'
                },
                {
                    id: 'V012',
                    type: 'Race Condition',
                    severity: 'High',
                    location: 'src/concurrency/locks.mo:53',
                    detectedDate: '2025-04-02T11:20:15Z',
                    description: 'Race condition in database transaction processing.',
                    recommendation: 'Implement proper locking mechanisms and transaction isolation.'
                },
                {
                    id: 'V013',
                    type: 'Integer Overflow',
                    severity: 'Medium',
                    location: 'src/index/counter.mo:74',
                    detectedDate: '2025-04-02T11:20:15Z',
                    description: 'Potential integer overflow in index counters.',
                    recommendation: 'Use safe math operations for counter increments.'
                },
                {
                    id: 'V014',
                    type: 'Memory Leak',
                    severity: 'Low',
                    location: 'src/cache/lru.mo:128',
                    detectedDate: '2025-04-02T11:20:15Z',
                    description: 'Memory leak in LRU cache implementation.',
                    recommendation: 'Properly clean up cached resources when they are evicted.'
                }
            ]
        }
    ]);
};
</script>

<template>
    <div class="card">
        <div class="font-semibold text-xl mb-4">Canister Security Reports</div>
        <DataTable v-model:expandedRows="expandedRows" 
                 :value="reports" 
                 dataKey="id" 
                 tableStyle="min-width: 60rem"
                 :loading="loading"
                 :filters="filters"
                 filterDisplay="menu"
                 :paginator="true" 
                 :rows="10"
                 paginatorTemplate="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink CurrentPageReport RowsPerPageDropdown"
                 :rowsPerPageOptions="[5, 10, 25]"
                 currentPageReportTemplate="Showing {first} to {last} of {totalRecords} reports">
            <template #header>
                <div class="flex flex-wrap justify-between">
                    <div>
                        <span class="p-input-icon-left">
                            <i class="pi pi-search" />
                            <InputText v-model="filters['global'].value" placeholder="Search..." />
                        </span>
                    </div>
                    <div class="flex gap-2">
                        <Button text icon="pi pi-plus" label="Expand All" @click="expandAll" />
                        <Button text icon="pi pi-minus" label="Collapse All" @click="collapseAll" />
                    </div>
                </div>
            </template>
            
            <Column expander style="width: 5rem" />
            <Column field="canisterId" header="Canister ID" sortable style="min-width: 12rem"></Column>
            <Column field="name" header="Canister Name" sortable style="min-width: 12rem"></Column>
            <Column field="scanDate" header="Scan Date" sortable style="min-width: 12rem">
                <template #body="slotProps">
                    {{ formatDate(slotProps.data.scanDate) }}
                </template>
                <template #filter="{ filterModel }">
                    <Calendar v-model="filterModel.value" dateFormat="mm/dd/yy" placeholder="mm/dd/yyyy" />
                </template>
            </Column>
            <Column field="type" header="Type" sortable style="min-width: 10rem"></Column>
            <Column field="vulnerabilityCount" header="Issues Found" sortable style="min-width: 8rem">
                <template #body="slotProps">
                    <Badge :value="slotProps.data.vulnerabilityCount" 
                           :severity="slotProps.data.vulnerabilityCount > 0 ? 'danger' : 'success'" />
                </template>
            </Column>
            <Column field="status" header="Status" sortable style="min-width: 10rem">
                <template #body="slotProps">
                    <Tag :value="slotProps.data.status" :severity="getSeverity(slotProps.data.status)" />
                </template>
            </Column>
            <Column field="scanDuration" header="Duration" sortable style="min-width: 8rem"></Column>
            <Column header="Actions" style="min-width: 10rem">
                <template #body>
                    <Button icon="pi pi-eye" class="p-button-rounded p-button-text" title="View Report" />
                    <Button icon="pi pi-download" class="p-button-rounded p-button-text" title="Download PDF" />
                </template>
            </Column>

            <template #expansion="slotProps">
                <div class="p-4">
                    <h5 class="mb-4">Vulnerabilities for {{ slotProps.data.name }}</h5>
                    
                    <div v-if="slotProps.data.vulnerabilities.length === 0" class="p-4 text-center">
                        <i class="pi pi-check-circle text-4xl text-green-500 mb-3"></i>
                        <div class="text-xl font-medium text-green-600">No vulnerabilities detected</div>
                        <p class="text-gray-600 mt-2">This canister passed all security checks.</p>
                    </div>
                    
                    <DataTable v-else :value="slotProps.data.vulnerabilities">
                        <Column field="id" header="ID" sortable style="width: 5rem"></Column>
                        <Column field="type" header="Type" sortable></Column>
                        <Column field="severity" header="Severity" sortable>
                            <template #body="slotProps">
                                <Tag :value="slotProps.data.severity" :severity="getVulnerabilitySeverity(slotProps.data.severity)" />
                            </template>
                        </Column>
                        <Column field="location" header="Location" sortable></Column>
                        <Column field="description" header="Description" sortable></Column>
                        <Column field="recommendation" header="Recommendation"></Column>
                    </DataTable>
                </div>
            </template>
            
            <template #empty>
                <div class="p-4 text-center">No scan reports found</div>
            </template>
            
            <template #loading>
                <div class="p-4 text-center">Loading canister security reports...</div>
            </template>
        </DataTable>
    </div>
</template>