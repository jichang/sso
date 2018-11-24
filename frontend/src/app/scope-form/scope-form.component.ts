import { Component, OnInit, Output, EventEmitter } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import { Router, ActivatedRoute } from "@angular/router";
import { Scope, ScopeModelService } from "../scope-model.service";
import { session } from "../model";
import { HttpErrorResponse } from "@angular/common/http";

@Component({
  selector: "scope-form",
  templateUrl: "./scope-form.component.html",
  styleUrls: ["./scope-form.component.css"]
})
export class ScopeFormComponent implements OnInit {
  scope: FormGroup;
  @Output() success = new EventEmitter();
  @Output() failure = new EventEmitter();

  constructor(
    private fb: FormBuilder,
    private scopeModelService: ScopeModelService,
    private router: Router,
    private route: ActivatedRoute
  ) {
    this.scope = fb.group({
      name: ["", [Validators.required]],
      description: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  create({ value, valid }: { value: Scope; valid: boolean }) {
    let currUser = session.currUser();
    if (currUser) {
      let applicationId = this.route.snapshot.params["id"];
      this.scopeModelService
        .create(currUser.id, applicationId, value)
        .subscribe(
          (response: any) => {
            this.success.emit(response);
          },
          (err: HttpErrorResponse) => {
            this.failure.emit(err);
          }
        );
    } else {
      this.router.navigate(["signin"]);
    }
  }
}
