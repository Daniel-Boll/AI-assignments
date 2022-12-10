export const sleep = (interval: number) =>
  new Promise((res) => setTimeout(res, interval));
