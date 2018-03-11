import { Pipe, PipeTransform } from "@angular/core";

@Pipe({
  name: "contactState"
})
export class ContactStatePipe implements PipeTransform {
  transform(value: number, args?: any): any {
    let txt = "";
    switch (value) {
      case 0:
        txt = "未认证";
    }

    return txt;
  }
}
