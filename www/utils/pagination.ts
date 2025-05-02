type GetPaginationPropsArgs = {
  currentPage: number;
  totalOfPages: number;
};

type GeneratePaginationButtonsArgs = {
  maxLeft: number;
  maxRight: number;
};

type ResolvePageUriArgs = {
  uri: string;
  page: number;
};

export type CalculatedPaginationData = {
  calculatedPages: number[];
  isFirstPageVisible: boolean;
  isLastPageVisible: boolean;
};

export abstract class PaginationUtils {
  private static visibleButtons = 6;

  public static resolvePageUri({ page, uri }: ResolvePageUriArgs): string {
    const url = new URL(uri, window.location.href);
    url.searchParams.set("page", page.toString());

    return url.toString();
  }

  public static getCalculatedPaginationData(
    props: GetPaginationPropsArgs,
  ): CalculatedPaginationData {
    const { isFirstPageVisible, isLastPageVisible, ...boundaries } =
      PaginationUtils.getPaginationProps(props);
    const { calculatedPages } =
      PaginationUtils.generatePaginationButtons(boundaries);

    return {
      calculatedPages,
      isFirstPageVisible,
      isLastPageVisible,
    };
  }

  private static generatePaginationButtons({
    maxLeft,
    maxRight,
  }: GeneratePaginationButtonsArgs) {
    const calculatedPages = [];

    console.log(maxLeft, maxRight);

    for (let page = maxLeft; page <= maxRight; ++page) {
      calculatedPages.push(page);
    }

    return { calculatedPages };
  }

  private static getPaginationProps({
    currentPage,
    totalOfPages,
  }: GetPaginationPropsArgs) {
    let maxLeft = currentPage - Math.floor(PaginationUtils.visibleButtons / 2);
    let maxRight = currentPage + Math.floor(PaginationUtils.visibleButtons / 2);
    let isFirstPageVisible = false;
    let isLastPageVisible = false;

    if (maxLeft <= 1) {
      maxLeft = 1;
      maxRight = PaginationUtils.visibleButtons;
    }

    if (maxRight >= totalOfPages) {
      maxLeft = totalOfPages - (PaginationUtils.visibleButtons - 1);
      maxRight = totalOfPages;
    }

    if (totalOfPages < PaginationUtils.visibleButtons + 1) {
      maxLeft = 1;
      maxRight = totalOfPages;
    }

    if (maxLeft <= 1) isFirstPageVisible = true;
    if (maxRight === totalOfPages) isLastPageVisible = true;

    return {
      maxLeft,
      maxRight,
      isFirstPageVisible,
      isLastPageVisible,
    };
  }
}
