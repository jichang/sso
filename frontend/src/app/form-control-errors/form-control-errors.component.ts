import {
  Component,
  Input,
  ElementRef,
  QueryList,
  ContentChildren,
  AfterContentInit,
  OnDestroy
} from "@angular/core";
import { FormGroupDirective, AbstractControl } from "@angular/forms";
import { FormControlErrorComponent } from "../form-control-error/form-control-error.component";
import { Subscription } from "rxjs";
import { map } from "rxjs/operators";

@Component({
  selector: "form-control-errors",
  templateUrl: "./form-control-errors.component.html",
  styleUrls: ["./form-control-errors.component.css"]
})
export class FormControlErrorsComponent implements AfterContentInit {
  @Input() for: string;
  @ContentChildren(FormControlErrorComponent)
  errorChildren: QueryList<ElementRef>;
  private control: AbstractControl;
  private subscription: Subscription;

  constructor(private form: FormGroupDirective) {}

  ngAfterContentInit() {
    this.control = this.form.control.get(this.for);
    this.subscription = this.control.statusChanges
      .pipe(
        map(status => {
          return [this.errorChildren, this.control.errors];
        })
      )
      .subscribe(([elementRefs, errors]) => {
        elementRefs.map(elementRef => {
          if (errors) {
            elementRef.update(!errors[elementRef.errKey]);
          } else {
            elementRef.update(true);
          }
        });
      });
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }
}
