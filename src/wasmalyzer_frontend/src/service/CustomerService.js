export const CustomerService = {
  getData() {
    return [
      {
        id: 1000,
        name: "CanisterBeta",
        country: {
          name: "Spyware",
          code: "us",
        },
        company: "Ransomware",
        date: "2017-01-18",
        status: "medium",
        verified: true,
        activity: 1,
        representative: {
          name: "John Doe",
          image: "johndoe.png",
        },
        balance: 93807,
      },
      {
        id: 1001,
        name: "CanisterDelta",
        country: {
          name: "Trojan Horse",
          code: "ru",
        },
        company: "Malware",
        date: "2018-02-19",
        status: "neutralized",
        verified: false,
        activity: 39,
        representative: {
          name: "Amy Elsner",
          image: "davidsmith.png",
        },
        balance: 96010,
      },
      {
        id: 1002,
        name: "CanisterGamma",
        country: {
          name: "Trojan Horse",
          code: "ru",
        },
        company: "Spyware",
        date: "2017-07-20",
        status: "mitigated",
        verified: false,
        activity: 60,
        representative: {
          name: "Michael Johnson",
          image: "davidsmith.png",
        },
        balance: 98016,
      },
      {
        id: 1003,
        name: "CanisterDelta",
        country: {
          name: "Malware",
          code: "bx",
        },
        company: "Trojan Horse",
        date: "2016-07-27",
        status: "medium",
        verified: false,
        activity: 34,
        representative: {
          name: "Lisa Ray",
          image: "davidsmith.png",
        },
        balance: 85231,
      },
      {
        id: 1004,
        name: "CanisterEpsilon",
        country: {
          name: "Ransomware",
          code: "us",
        },
        company: "Trojan Horse",
        date: "2017-08-08",
        status: "high",
        verified: false,
        activity: 15,
        representative: {
          name: "John Doe",
          image: "amyelsner.png",
        },
        balance: 62545,
      },
      {
        id: 1005,
        name: "CanisterEpsilon",
        country: {
          name: "Malware",
          code: "cz",
        },
        company: "Malware",
        date: "2015-12-20",
        status: "neutralized",
        verified: true,
        activity: 99,
        representative: {
          name: "John Doe",
          image: "davidsmith.png",
        },
        balance: 73362,
      },
      {
        id: 1006,
        name: "CanisterEpsilon",
        country: {
          name: "Trojan Horse",
          code: "ru",
        },
        company: "Ransomware",
        date: "2021-04-03",
        status: "high",
        verified: false,
        activity: 72,
        representative: {
          name: "Michael Johnson",
          image: "davidsmith.png",
        },
        balance: 479,
      },
    ];
  },

  getCustomersSmall() {
    return Promise.resolve(this.getData().slice(0, 10));
  },

  getCustomersMedium() {
    return Promise.resolve(this.getData().slice(0, 50));
  },

  getCustomersLarge() {
    return Promise.resolve(this.getData().slice(0, 200));
  },

  getCustomersXLarge() {
    return Promise.resolve(this.getData());
  },

  getCustomers(params) {
    const queryParams = params
      ? Object.keys(params)
          .map(
            (k) => encodeURIComponent(k) + "=" + encodeURIComponent(params[k])
          )
          .join("&")
      : "";

    return fetch(
      "https://www.primefaces.org/data/customers?" + queryParams
    ).then((res) => res.json());
  },
};
