import { Component, OnInit, Output, EventEmitter } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { User } from "../model";

@Component({
  selector: "signin-form",
  templateUrl: "./signin-form.component.html",
  styleUrls: ["./signin-form.component.css"]
})
export class SigninFormComponent implements OnInit {
  user: FormGroup;
  @Output() success = new EventEmitter();
  @Output() failure = new EventEmitter();

  constructor(private fb: FormBuilder, private httpClient: HttpClient) {
    this.user = fb.group({
      username: ["", [Validators.required]],
      password: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  signin({ value, valid }: { value: User; valid: boolean }) {
    this.httpClient.post("/api/v1/signin", value).subscribe(
      (response: Response) => {
        this.success.emit(response);
      },
      (err: HttpErrorResponse) => {
        this.failure.emit(err);
      }
    );
  }
}
