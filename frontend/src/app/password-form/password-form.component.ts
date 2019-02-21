import { Component, OnInit, EventEmitter, Output } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import {
  HttpClient,
  HttpHeaders,
  HttpErrorResponse
} from "@angular/common/http";
import { Router, ActivatedRoute } from "@angular/router";
import { Profile, GenderType, Gender, session } from "../model";

@Component({
  selector: "password-form",
  templateUrl: "./password-form.component.html",
  styleUrls: ["./password-form.component.css"]
})
export class PasswordFormComponent implements OnInit {
  password: FormGroup;
  @Output() success = new EventEmitter();
  @Output() failure = new EventEmitter();

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.password = fb.group({
      old_password: ["", [Validators.required]],
      new_password: ["", [Validators.required]],
      confirm_password: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  update({ value, valid }: { value: any; valid: boolean }) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = "/api/v1/users/" + user.id + "/password";
    let params = {
      old_password: value.old_password,
      new_password: value.new_password
    };
    this.http.post(apiUri, params, options).subscribe(
      (response: Response) => {
        this.success.emit(response);
      },
      (err: HttpErrorResponse) => {
        this.failure.emit(err);
      }
    );
  }
}
