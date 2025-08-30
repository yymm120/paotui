export interface WorkInfoTody {
  incomeToday: number;
  orderCountTody: number;
}

export const useWorkInfoToday = (): WorkInfoTody => {
  return {
    incomeToday: 9.99,
    orderCountTody: 9,
  };
};
