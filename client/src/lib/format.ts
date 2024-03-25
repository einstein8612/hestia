export const formatTimeSinceDate = (date: Date): string => {
    return formatTimeBetweenDates(date, new Date());
}

export const formatTimeBetweenDates = (before: Date, after: Date): string => {
    const yearsPassed = after.getFullYear() - before.getFullYear();
    if (yearsPassed > 1) {
        return yearsPassed + " years ago";
    }
    if (yearsPassed > 0) {
        return yearsPassed + " year ago";
    }

    const monthPassed = after.getDate() >= before.getDate() ? 0 : 1;
    const monthsPassed = yearsPassed*12 + after.getMonth() - before.getMonth() - monthPassed;
    if (monthsPassed > 1) {
        return monthsPassed + " months ago";
    }
    if (monthsPassed > 0) {
        return monthsPassed + " month ago";
    }

    const daysPassed = Math.floor((after.getTime() - before.getTime()) / (1000*60*60*24));
    if (daysPassed > 1) {
        return daysPassed + " days ago";
    }
    if (daysPassed > 0 || before.getDate() !== after.getDate()) {
        return "Yesterday";
    }
    return "Today";
}

export const formatLargeNumber = (num?: number): string => {
    if (num === undefined) return "NaN";
    if (num < 1000) {
        return num.toString();
    }
    num /= 1000;
    if (num < 1000) {
        return roundToString(num, 2)+"k";
    }
    num /= 1000;
    if (num < 1000) {
        return roundToString(num, 2)+"M";
    }
    num /= 1000;
    return roundToString(num, 2)+"B";
}

export const formatStorageNumber = (num?: number): string => {
    if (num === undefined) return "NaN";
    if (num < 1) {
        return roundToString(num*1024, 3)+"KiB";
    }
    if (num < 1024) {
        return roundToString(num, 3)+"MiB";
    }
    num /= 1024;
    if (num < 1024) {
        return roundToString(num, 3)+"GiB";
    }
    num /= 1024;
    return roundToString(num, 3)+"TiB";
}

const roundToString = (num: number, digits: number): string => (Math.floor(num * Math.pow(10, digits)) / Math.pow(10, digits)).toFixed(digits)