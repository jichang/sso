import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'mask'
})
export class MaskPipe implements PipeTransform {

  transform(value: any, args?: any): any {
    if (!value) {
      return '*';
    }

    let count = args;
    return value.slice(0, count) + '****' + value.slice(-count, value.length);
  }

}
