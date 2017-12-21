import { Component, OnInit } from '@angular/core';
import { Application, ApplicationModelService } from '../application-model.service';
import { session } from '../model';

@Component({
  selector: 'applications-page',
  templateUrl: './applications-page.component.html',
  styleUrls: ['./applications-page.component.css']
})
export class ApplicationsPageComponent implements OnInit {
  applications: Application[] = [];

  constructor(private applicationModel: ApplicationModelService) { }

  ngOnInit() {
    this.applicationModel.applications.subscribe(applications => {
        this.applications = applications;
      });

    this.applicationModel.select();
  }

  openCreateModal() {}
}
