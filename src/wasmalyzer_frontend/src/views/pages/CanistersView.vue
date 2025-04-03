<script setup>
import { CanisterService } from '@/service/CanisterService';
import { FilterMatchMode } from '@primevue/core/api';
import { useToast } from 'primevue/usetoast';
import { onMounted, ref } from 'vue';

onMounted(() => {
    CanisterService.getCanisters().then((data) => (canisters.value = data));
});

const toast = useToast();
const dt = ref();
const canisters = ref();
const canisterDialog = ref(false);
const deleteCanisterDialog = ref(false);
const deleteCanistersDialog = ref(false);
const scanCanisterDialog = ref(false);
const scanningInProgress = ref(false);
const scanResult = ref(null);
const canister = ref({});
const selectedCanisters = ref();
const filters = ref({
    global: { value: null, matchMode: FilterMatchMode.CONTAINS }
});
const submitted = ref(false);
const statuses = ref([
    { label: 'RUNNING', value: 'running' },
    { label: 'STOPPED', value: 'stopped' },
    { label: 'FAILED', value: 'failed' }
]);

// Scan vulnerabilities options
const vulnerabilityTypes = [
    'Buffer Overflow',
    'Integer Overflow',
    'Use After Free',
    'Memory Leak',
    'Race Condition',
    'Uninitialized Memory',
    'Stack Overflow',
    'Heap Corruption',
    'Null Pointer Dereference',
    'Infinite Loop'
];

const severityLevels = ['Critical', 'High', 'Medium', 'Low', 'Informational'];

function formatCycles(value) {
    if (value) return value.toLocaleString('en-US');
    return;
}

function openNew() {
    canister.value = {};
    submitted.value = false;
    canisterDialog.value = true;
}

function hideDialog() {
    canisterDialog.value = false;
    submitted.value = false;
}

function saveCanister() {
    submitted.value = true;

    if (canister?.value.name?.trim()) {
        if (canister.value.id) {
            canister.value.status = canister.value.status.value ? canister.value.status.value : canister.value.status;
            canisters.value[findIndexById(canister.value.id)] = canister.value;
            toast.add({ severity: 'success', summary: 'Successful', detail: 'Canister Updated', life: 3000 });
        } else {
            canister.value.id = createId();
            canister.value.icon = 'canister-placeholder.svg';
            canister.value.status = canister.value.status ? canister.value.status.value : 'RUNNING';
            canisters.value.push(canister.value);
            toast.add({ severity: 'success', summary: 'Successful', detail: 'Canister Created', life: 3000 });
        }

        canisterDialog.value = false;
        canister.value = {};
    }
}

function editCanister(can) {
    canister.value = { ...can };
    canisterDialog.value = true;
}

function confirmDeleteCanister(can) {
    canister.value = can;
    deleteCanisterDialog.value = true;
}

function deleteCanister() {
    canisters.value = canisters.value.filter((val) => val.id !== canister.value.id);
    deleteCanisterDialog.value = false;
    canister.value = {};
    toast.add({ severity: 'success', summary: 'Successful', detail: 'Canister Deleted', life: 3000 });
}

function findIndexById(id) {
    let index = -1;
    for (let i = 0; i < canisters.value.length; i++) {
        if (canisters.value[i].id === id) {
            index = i;
            break;
        }
    }

    return index;
}

function createId() {
    let id = '';
    var chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (var i = 0; i < 5; i++) {
        id += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return id;
}

function exportCSV() {
    dt.value.exportCSV();
}

function confirmDeleteSelected() {
    deleteCanistersDialog.value = true;
}

function deleteSelectedCanisters() {
    canisters.value = canisters.value.filter((val) => !selectedCanisters.value.includes(val));
    deleteCanistersDialog.value = false;
    selectedCanisters.value = null;
    toast.add({ severity: 'success', summary: 'Successful', detail: 'Canisters Deleted', life: 3000 });
}

function getStatusLabel(status) {
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

function getSeverityLabel(severity) {
    switch (severity) {
        case 'Critical':
            return 'danger';
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

function getRandomInt(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function scanCanister(can) {
    canister.value = { ...can };
    scanCanisterDialog.value = true;
    scanningInProgress.value = true;
    scanResult.value = null;
    
    // Simulate scanning for 10 seconds then generate results based on canister status
    setTimeout(() => {
        scanningInProgress.value = false;
        
        // Generate random number of vulnerabilities based on status
        // RUNNING = 0 vulnerabilities (secure)
        // STOPPED or FAILED = 1-5 vulnerabilities (issues found)
        const numberOfVulnerabilities = canister.value.status === 'RUNNING' ? 0 : getRandomInt(1, 5);
        const vulnerabilities = [];
        
        // Generate fake vulnerabilities
        for (let i = 0; i < numberOfVulnerabilities; i++) {
            const vulnType = vulnerabilityTypes[getRandomInt(0, vulnerabilityTypes.length - 1)];
            const severity = severityLevels[getRandomInt(0, severityLevels.length - 1)];
            const lineNumber = getRandomInt(10, 500);
            
            vulnerabilities.push({
                id: createId(),
                type: vulnType,
                severity: severity,
                location: `src/lib.rs:${lineNumber}`,
                description: `Potential ${vulnType.toLowerCase()} vulnerability detected in the canister code.`,
                recommendation: `Review the code at line ${lineNumber} to ensure proper input validation and memory management.`
            });
        }
        
        // Generate scan result summary
        scanResult.value = {
            timestamp: new Date().toISOString(),
            canisterId: canister.value.canisterId,
            canisterName: canister.value.name,
            vulnerabilities: vulnerabilities,
            memoryUsage: `${getRandomInt(10, canister.value.memorySize)}MB / ${canister.value.memorySize}MB`,
            cyclesUsed: getRandomInt(1000, 10000),
            scanDuration: `${getRandomInt(8, 12)}.${getRandomInt(100, 999)}s`,
            wasmSize: `${getRandomInt(100, 1000)}KB`
        };
        
        // Show toast notification with scan result summary
        const severity = numberOfVulnerabilities > 0 ? 
            (numberOfVulnerabilities > 3 ? 'error' : 'warn') : 
            'success';
            
        let message;
        if (numberOfVulnerabilities === 0) {
            message = `No vulnerabilities detected in ${canister.value.name}. Canister is secure!`;
        } else {
            message = `Found ${numberOfVulnerabilities} potential vulnerabilities in ${canister.value.name}`;
        }
        
        toast.add({
            severity: severity,
            summary: 'Scan Completed',
            detail: message,
            life: 5000
        });
    }, 10000);
}
</script>

<template>
    <div>
        <div class="card">
            <Toolbar class="mb-6">
                <template #start>
                    <Button label="New" icon="pi pi-plus" severity="secondary" class="mr-2" @click="openNew" />
                    <Button label="Delete" icon="pi pi-trash" severity="secondary" @click="confirmDeleteSelected" :disabled="!selectedCanisters || !selectedCanisters.length" />
                </template>

                <template #end>
                    <Button label="Export" icon="pi pi-upload" severity="secondary" @click="exportCSV($event)" />
                </template>
            </Toolbar>

            <DataTable
                ref="dt"
                v-model:selection="selectedCanisters"
                :value="canisters"
                dataKey="id"
                :paginator="true"
                :rows="10"
                :filters="filters"
                paginatorTemplate="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink CurrentPageReport RowsPerPageDropdown"
                :rowsPerPageOptions="[5, 10, 25]"
                currentPageReportTemplate="Showing {first} to {last} of {totalRecords} canisters"
            >
                <template #header>
                    <div class="flex flex-wrap gap-2 items-center justify-between">
                        <h4 class="m-0">Manage Canisters</h4>
                        <IconField>
                            <InputIcon>
                                <i class="pi pi-search" />
                            </InputIcon>
                            <InputText v-model="filters['global'].value" placeholder="Search..." />
                        </IconField>
                    </div>
                </template>

                <Column selectionMode="multiple" style="width: 3rem" :exportable="false"></Column>
                <Column field="canisterId" header="Canister ID" sortable style="min-width: 12rem"></Column>
                <Column field="name" header="Name" sortable style="min-width: 16rem"></Column>
                <Column header="Icon">
                    <template #body="slotProps">
                        <img :src="`/assets/icons/${slotProps.data.icon}`" :alt="slotProps.data.icon" class="rounded" style="width: 64px" />
                    </template>
                </Column>
                <Column field="cycles" header="Cycles" sortable style="min-width: 8rem">
                    <template #body="slotProps">
                        {{ formatCycles(slotProps.data.cycles) }}
                    </template>
                </Column>
                <Column field="type" header="Type" sortable style="min-width: 10rem"></Column>
                <Column field="memorySize" header="Memory (MB)" sortable style="min-width: 12rem"></Column>
                <Column field="status" header="Status" sortable style="min-width: 12rem">
                    <template #body="slotProps">
                        <Tag :value="slotProps.data.status" :severity="getStatusLabel(slotProps.data.status)" />
                    </template>
                </Column>
                <Column :exportable="false" style="min-width: 14rem">
                    <template #body="slotProps">
                        <Button icon="pi pi-pencil" outlined rounded class="mr-2" @click="editCanister(slotProps.data)" />
                        <Button icon="pi pi-trash" outlined rounded severity="danger" class="mr-2" @click="confirmDeleteCanister(slotProps.data)" />
                        <Button icon="pi pi-search" outlined rounded severity="info" @click="scanCanister(slotProps.data)" />
                    </template>
                </Column>
            </DataTable>
        </div>

        <Dialog v-model:visible="canisterDialog" :style="{ width: '450px' }" header="Canister Details" :modal="true">
            <div class="flex flex-col gap-6">
                <img v-if="canister.icon" :src="`/assets/icons/${canister.icon}`" :alt="canister.icon" class="block m-auto pb-4" />
                <div>
                    <label for="name" class="block font-bold mb-3">Name</label>
                    <InputText id="name" v-model.trim="canister.name" required="true" autofocus :invalid="submitted && !canister.name" fluid />
                    <small v-if="submitted && !canister.name" class="text-red-500">Name is required.</small>
                </div>
                <div>
                    <label for="canisterId" class="block font-bold mb-3">Canister ID</label>
                    <InputText id="canisterId" v-model.trim="canister.canisterId" placeholder="e.g. rrkah-fqaaa-aaaaa-aaaaq-cai" :disabled="!!canister.id" />
                    <small class="text-gray-500">Canister IDs are immutable after creation</small>
                </div>
                <div>
                    <label for="description" class="block font-bold mb-3">Description</label>
                    <Textarea id="description" v-model="canister.description" required="true" rows="3" cols="20" fluid />
                </div>
                <div>
                    <label for="status" class="block font-bold mb-3">Status</label>
                    <Select id="status" v-model="canister.status" :options="statuses" optionLabel="label" placeholder="Select a Status" fluid></Select>
                </div>

                <div>
                    <span class="block font-bold mb-4">Type</span>
                    <div class="grid grid-cols-12 gap-4">
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="type1" v-model="canister.type" name="type" value="Smart Contract" />
                            <label for="type1">Smart Contract</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="type2" v-model="canister.type" name="type" value="Asset Storage" />
                            <label for="type2">Asset Storage</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="type3" v-model="canister.type" name="type" value="Database" />
                            <label for="type3">Database</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="type4" v-model="canister.type" name="type" value="Backend Service" />
                            <label for="type4">Backend Service</label>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-6">
                        <label for="cycles" class="block font-bold mb-3">Cycles</label>
                        <InputNumber id="cycles" v-model="canister.cycles" integeronly fluid />
                    </div>
                    <div class="col-span-6">
                        <label for="memorySize" class="block font-bold mb-3">Memory (MB)</label>
                        <InputNumber id="memorySize" v-model="canister.memorySize" integeronly fluid />
                    </div>
                </div>
            </div>

            <template #footer>
                <Button label="Cancel" icon="pi pi-times" text @click="hideDialog" />
                <Button label="Save" icon="pi pi-check" @click="saveCanister" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteCanisterDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="canister"
                    >Are you sure you want to delete <b>{{ canister.name }}</b
                    >?</span
                >
            </div>
            <template #footer>
                <Button label="No" icon="pi pi-times" text @click="deleteCanisterDialog = false" />
                <Button label="Yes" icon="pi pi-check" @click="deleteCanister" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteCanistersDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="canister">Are you sure you want to delete the selected canisters?</span>
            </div>
            <template #footer>
                <Button label="No" icon="pi pi-times" text @click="deleteCanistersDialog = false" />
                <Button label="Yes" icon="pi pi-check" text @click="deleteSelectedCanisters" />
            </template>
        </Dialog>

        <Dialog v-model:visible="scanCanisterDialog" :style="{ width: '650px' }" header="Canister Security Scanner" :modal="true" :closable="!scanningInProgress">
            <div class="flex flex-col gap-6">
                <div v-if="scanningInProgress" class="flex flex-col items-center justify-center py-6">
                    <ProgressSpinner style="width:100px;height:100px" strokeWidth="5" />
                    <h3 class="mt-4">Scanning canister {{ canister.name }}...</h3>
                    <p class="text-center text-gray-500 mt-2">
                        Analyzing bytecode, checking memory safety, validating state transitions,<br />
                        and scanning for security vulnerabilities. Please wait...
                    </p>
                </div>
                
                <div v-else-if="scanResult">
                    <div class="mb-4 p-3 surface-0 border-round">
                        <div class="grid">
                            <div class="col-12 md:col-6">
                                <div class="mb-3">
                                    <span class="font-bold">Canister ID:</span>
                                    <span class="ml-2">{{ scanResult.canisterId }}</span>
                                </div>
                                <div class="mb-3">
                                    <span class="font-bold">Scan Duration:</span>
                                    <span class="ml-2">{{ scanResult.scanDuration }}</span>
                                </div>
                            </div>
                            <div class="col-12 md:col-6">
                                <div class="mb-3">
                                    <span class="font-bold">Memory Usage:</span>
                                    <span class="ml-2">{{ scanResult.memoryUsage }}</span>
                                </div>
                                <div class="mb-3">
                                    <span class="font-bold">Wasm Size:</span>
                                    <span class="ml-2">{{ scanResult.wasmSize }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <Divider />
                    
                    <h3>Scan Results</h3>
                    <p v-if="!scanResult.vulnerabilities || scanResult.vulnerabilities.length === 0" class="text-green-500 font-bold">
                        No vulnerabilities detected! Canister appears secure.
                    </p>
                    
                    <DataTable v-else :value="scanResult.vulnerabilities" stripedRows>
                        <Column field="type" header="Vulnerability Type" />
                        <Column field="severity" header="Severity">
                            <template #body="slotProps">
                                <Tag :value="slotProps.data.severity" :severity="getSeverityLabel(slotProps.data.severity)" />
                            </template>
                        </Column>
                        <Column field="location" header="Location" />
                        <Column field="description" header="Description" />
                    </DataTable>
                </div>
            </div>

            <template #footer>
                <Button label="Close" icon="pi pi-times" text @click="scanCanisterDialog = false" :disabled="scanningInProgress" />
                <Button v-if="scanResult && scanResult.vulnerabilities && scanResult.vulnerabilities.length > 0" 
                       label="Generate Detailed Report" 
                       icon="pi pi-file" 
                       class="ml-2" />
            </template>
        </Dialog>
    </div>
</template>
