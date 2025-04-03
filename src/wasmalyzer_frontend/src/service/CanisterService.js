export const CanisterService = {
    getCanisters() {
        return fetch('/data/canisters.json')
            .then(res => res.json())
            .then(d => d.data)
            .catch(error => {
                console.error('Error fetching canisters:', error);
                return []; // Return empty array if fetch fails
            });
    }
};