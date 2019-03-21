import { Component, OnInit, Output, EventEmitter } from "@angular/core";
import { Validators, FormGroup, FormBuilder } from "@angular/forms";
import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { User } from "../model";

@Component({
  selector: "totp-form",
  templateUrl: "./totp-form.component.html",
  styleUrls: ["./totp-form.component.css"]
})
export class TotpFormComponent implements OnInit {
  totp: FormGroup;
  @Output() success = new EventEmitter();
  @Output() failure = new EventEmitter();

  constructor(private fb: FormBuilder, private httpClient: HttpClient) {
    this.totp = fb.group({
      code: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  verify({ value, valid }: { value: { code: string }; valid: boolean }) {
    this.httpClient
      .post("/api/v1/signin/totp", {
        code: parseInt(value.code, 10)
      })
      .subscribe(
        (response: Response) => {
          this.success.emit(response);
        },
        (err: HttpErrorResponse) => {
          this.failure.emit(err);
        }
      );
  }
}
