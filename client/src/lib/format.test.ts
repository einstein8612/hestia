import { describe, it, expect } from 'vitest';
import { formatLargeNumber, formatStorageNumber, formatTimeBetweenDates, formatTimeSinceDate } from './format';

describe('between dates', () => {
	it('many years between', () => {
		expect(formatTimeBetweenDates(new Date(2000,1,1), new Date(2020,1,1)))
			.toBe("20 years ago");
	});

	it('exactly one year between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,1), new Date(2023,1,1)))
			.toBe("1 year ago");
	});

	it('exactly one year and one day between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,1), new Date(2023,1,2)))
			.toBe("1 year ago");
	});

	it('exactly two years minus one day between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,1), new Date(2023,11,31)))
			.toBe("1 year ago");
	});

	it('many months between', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,1), new Date(2022,11,1)))
			.toBe("11 months ago");
	});

	it('almost a month between', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,1), new Date(2022,0,31)))
			.toBe("30 days ago");
	});

	it('one month between', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,1), new Date(2022,1,1)))
			.toBe("1 month ago");
	});

	it('one month between in days', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,15), new Date(2022,1,15)))
			.toBe("1 month ago");
	});

	it('one month between in february', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,15), new Date(2022,2,15)))
			.toBe("1 month ago");
	});

	it('15 days between in february', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,1), new Date(2022,1,16)))
			.toBe("15 days ago");
	});

	it('exactly two months minus one day between in february', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,1), new Date(2022,1,28)))
			.toBe("1 month ago");
	});

	it('2 days between', () => {
		expect(formatTimeBetweenDates(new Date(2022,0,1), new Date(2022,0,3)))
			.toBe("2 days ago");
	});

	it('2 days between across months', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,28), new Date(2022,2,2)))
			.toBe("2 days ago");
	});

	it('10 days between across months', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,28), new Date(2022,2,10)))
			.toBe("10 days ago");
	});

	it('10 days between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,18), new Date(2022,1,28)))
			.toBe("10 days ago");
	});

	it('1 day between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,18), new Date(2022,1,19)))
			.toBe("Yesterday");
	});

	it('1 day between different months', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,28), new Date(2022,2,1)))
			.toBe("Yesterday");
	});

	it('less than 1 day between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,18, 0, 0, 0, 0), new Date(2022,1,18, 23, 59, 59, 999)))
			.toBe("Today");
	});

	it('less than 24 hours but a day in between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,18, 17, 0, 0, 0), new Date(2022,1,19, 16, 59, 59, 999)))
			.toBe("Yesterday");
	});

	it('1 milliseconds but a day in between', () => {
		expect(formatTimeBetweenDates(new Date(2022,1,18, 23, 59, 59, 999), new Date(2022,1,19, 0, 0, 0, 0)))
			.toBe("Yesterday");
	});
});


describe('since date', () => {
	it('simple test yesterday', () => {
		expect(formatTimeSinceDate(new Date()))
			.toBe("Today");
	});

	it('simple test yesterday', () => {
		expect(formatTimeSinceDate(new Date(Date.now() - 1000*60*60*24)))
			.toBe("Yesterday");
	});

	it('simple test month', () => {
		expect(formatTimeSinceDate(new Date(Date.now() - 1000*60*60*24*31)))
			.toBe("1 month ago");
	});

	it('simple test year', () => {
		expect(formatTimeSinceDate(new Date(Date.now() - 1000*60*60*24*366)))
			.toBe("1 year ago");
	});
});

describe('large number', () => {
	it('simple number undefined', () => {
		expect(formatLargeNumber(undefined))
			.toBe("NaN");
	});
	it('simple number 0', () => {
		expect(formatLargeNumber(0))
			.toBe("0");
	});
	it('simple number below 1000', () => {
		expect(formatLargeNumber(100))
			.toBe("100");
	});
	it('simple number barely below 1000', () => {
		expect(formatLargeNumber(999))
			.toBe("999");
	});
	it('simple number 1000', () => {
		expect(formatLargeNumber(1000))
			.toBe("1.00k");
	});
	it('simple number always round down', () => {
		expect(formatLargeNumber(1009))
			.toBe("1.00k");
	});
	it('simple number rounded down', () => {
		expect(formatLargeNumber(1005))
			.toBe("1.00k");
	});
	it('simple number random in the thousands', () => {
		expect(formatLargeNumber(123712))
			.toBe("123.71k");
	});
	it('simple number barely below million', () => {
		expect(formatLargeNumber(999999))
			.toBe("999.99k");
	});
	it('simple number barely above million', () => {
		expect(formatLargeNumber(1000000))
			.toBe("1.00M");
	});
	it('simple number random in the millions', () => {
		expect(formatLargeNumber(9182842))
			.toBe("9.18M");
	});
	it('simple number barely below billion', () => {
		expect(formatLargeNumber(999999999))
			.toBe("999.99M");
	});
	it('simple number barely above billion', () => {
		expect(formatLargeNumber(1000000000))
			.toBe("1.00B");
	});
	it('simple number random in the billions', () => {
		expect(formatLargeNumber(4291823842))
			.toBe("4.29B");
	});
});

describe('storage number', () => {
	it('simple number undefined', () => {
		expect(formatStorageNumber(undefined))
			.toBe("NaN");
	});
	it('simple number 0', () => {
		expect(formatStorageNumber(0))
			.toBe("0.000KiB");
	});
	it('simple number random below 1', () => {
		expect(formatStorageNumber(0.91824))
			.toBe("940.277KiB");
	});
	it('simple number below 1G', () => {
		expect(formatStorageNumber(100))
			.toBe("100.000MiB");
	});
	it('simple number barely below 1G', () => {
		expect(formatStorageNumber(1023.9999))
			.toBe("1023.999MiB");
	});
	it('simple number 1G', () => {
		expect(formatStorageNumber(1024))
			.toBe("1.000GiB");
	});
	it('simple number random in the gigabytes', () => {
		expect(formatStorageNumber(123712))
			.toBe("120.812GiB");
	});
	it('simple number barely below terrabyte', () => {
		expect(formatStorageNumber(1048575.9999))
			.toBe("1023.999GiB");
	});
	it('simple number barely above terrabyte', () => {
		expect(formatStorageNumber(1048576))
			.toBe("1.000TiB");
	});
	it('simple number random in the terrabytes', () => {
		expect(formatStorageNumber(9182842))
			.toBe("8.757TiB");
	});
});